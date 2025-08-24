use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::time::Duration as StdDuration;
use chrono::{DateTime, Utc, Duration};
use tokio::time::interval;

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub requests_per_minute: Vec<DateTime<Utc>>,
    pub requests_per_12h: Vec<DateTime<Utc>>,
}

impl RateLimitInfo {
    pub fn new() -> Self {
        Self {
            requests_per_minute: Vec::new(),
            requests_per_12h: Vec::new(),
        }
    }

    pub fn can_make_request(&mut self) -> bool {
        let now = Utc::now();
        
        // Limpiar requests antiguos (más de 1 minuto)
        self.requests_per_minute.retain(|&time| {
            now.signed_duration_since(time) < Duration::minutes(1)
        });

        // Limpiar requests antiguos (más de 12 horas)
        self.requests_per_12h.retain(|&time| {
            now.signed_duration_since(time) < Duration::hours(12)
        });

        // Si no hay requests recientes, la IP puede hacer peticiones y se libera memoria
        if self.requests_per_minute.is_empty() && self.requests_per_12h.is_empty() {
            return true;
        }

        // Verificar límite por minuto (máximo 2)
        if self.requests_per_minute.len() >= 2 {
            return false;
        }

        // Verificar límite por 12 horas (máximo 4)
        if self.requests_per_12h.len() >= 4 {
            return false;
        }

        // Agregar el nuevo request
        self.requests_per_minute.push(now);
        self.requests_per_12h.push(now);

        true
    }

    pub fn get_remaining_requests(&self) -> (usize, usize) {
        let now = Utc::now();
        
        let minute_requests = self.requests_per_minute.iter()
            .filter(|&time| now.signed_duration_since(*time) < Duration::minutes(1))
            .count();
        
        let hour_12_requests = self.requests_per_12h.iter()
            .filter(|&time| now.signed_duration_since(*time) < Duration::hours(12))
            .count();

        (2 - minute_requests, 4 - hour_12_requests)
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, RateLimitInfo>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn check_rate_limit(&self, ip: &str) -> (bool, (usize, usize)) {
        let mut requests = self.requests.lock().unwrap();
        
        let rate_info = requests.entry(ip.to_string()).or_insert_with(RateLimitInfo::new);
        let can_request = rate_info.can_make_request();
        let remaining = rate_info.get_remaining_requests();

        // Si la IP puede hacer peticiones (no hay requests recientes), eliminarla de memoria
        if can_request && rate_info.requests_per_minute.is_empty() && rate_info.requests_per_12h.is_empty() {
            requests.remove(ip);
        }

        (can_request, remaining)
    }

    /// Limpia automáticamente las IPs que ya pueden hacer peticiones
    pub fn cleanup_expired_ips(&self) {
        let now = Utc::now();
        let mut requests = self.requests.lock().unwrap();
        let mut to_remove = Vec::new();

        for (ip, rate_info) in requests.iter() {
            // Verificar si la IP ya puede hacer peticiones (todos los timestamps son antiguos)
            let has_recent_minute_requests = rate_info.requests_per_minute.iter()
                .any(|&time| now.signed_duration_since(time) < Duration::minutes(1));
            
            let has_recent_12h_requests = rate_info.requests_per_12h.iter()
                .any(|&time| now.signed_duration_since(time) < Duration::hours(12));

            // Si no hay requests recientes, marcar para eliminar
            if !has_recent_minute_requests && !has_recent_12h_requests {
                to_remove.push(ip.clone());
            }
        }

        // Eliminar las IPs marcadas
        for ip in to_remove {
            requests.remove(&ip);
        }
    }

    /// Inicia la limpieza automática periódica
    pub async fn start_auto_cleanup(&self) {
        let rate_limiter = self.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(StdDuration::from_secs(86400)); // Limpiar cada 24 horas
            
            loop {
                interval.tick().await;
                rate_limiter.cleanup_expired_ips();
            }
        });
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
