use crate::models::DnsLog;
use chrono::Local;
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    AsyncResolver,
};
use hickory_server::{
    authority::MessageResponseBuilder,
    proto::op::Header,
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedState = Arc<RwLock<Vec<DnsLog>>>;

#[derive(Clone)]
pub struct DnsInspector {
    resolver: AsyncResolver<hickory_resolver::name_server::TokioConnectionProvider>,
    state: SharedState,
}

impl DnsInspector {
    pub fn new(state: SharedState) -> Self {
        let config = ResolverConfig::cloudflare_https();
        let mut opts = ResolverOpts::default();
        opts.use_hosts_file = false;

        let resolver = AsyncResolver::tokio(config, opts);
        Self { resolver, state }
    }
}

#[async_trait::async_trait]
impl RequestHandler for DnsInspector {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        let query = request.query();
        let name_str = query.name().to_string();
        let record_type = query.query_type();

        println!("🕵️ [Müfettiş Yakaladı] İstek: {} ---> Tipi: {:?}", name_str, record_type);

        let qname = hickory_resolver::Name::from_str(&name_str).unwrap();
        let resolve_result = self.resolver.lookup(qname, record_type).await;

        let status = if resolve_result.is_ok() {
            "Yakalandı & Çözüldü"
        } else {
            "Çözümleme Hatası"
        };

        {
            let mut logs = self.state.write().await;
            let id = logs.len() + 1;
            logs.push(DnsLog {
                id,
                timestamp: Local::now().format("%H:%M:%S").to_string(),
                domain: name_str.clone(),
                record_type: format!("{:?}", record_type),
                status: status.to_string(),
            });

            if logs.len() > 1000 {
                logs.remove(0);
            }
        }

        match resolve_result {
            Ok(lookup) => {
                let builder = MessageResponseBuilder::from_message_request(request);
                let mut header = Header::response_from_request(request.header());
                header.set_authoritative(false);

                let records: Vec<_> = lookup.record_iter().cloned().collect();
                let response = builder.build(
                    header,
                    records.iter(),
                    std::iter::empty(),
                    std::iter::empty(),
                    std::iter::empty(),
                );

                response_handle.send_response(response).await.unwrap_or_else(|_| {
                    let mut fallback_header = Header::response_from_request(request.header());
                    fallback_header.set_response_code(hickory_server::proto::op::ResponseCode::ServFail);
                    fallback_header.into()
                })
            }
            Err(_) => {
                let builder = MessageResponseBuilder::from_message_request(request);
                let mut header = Header::response_from_request(request.header());
                header.set_response_code(hickory_server::proto::op::ResponseCode::ServFail);
                let response = builder.build_no_records(header);
                response_handle.send_response(response).await.unwrap_or_else(|_| header.into())
            }
        }
    }
}