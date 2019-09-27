use crate::kv::KeyValue;
use crate::proto::rpc;
use crate::ResponseHeader;

pub struct DeleteRequest {
    key: Vec<u8>,
    end_key: Option<Vec<u8>>,
    prev_kv: bool,
}

impl DeleteRequest {
    pub fn key(key: &[u8]) -> Self
    {
        Self {
            key: key.to_vec(),
            end_key: None,
            prev_kv: false,
        }
    }

    pub fn range(key: &[u8], end_key: &[u8]) -> Self
    {
        Self {
            key: key.to_vec(),
            end_key: Some(end_key.to_vec()),
            prev_kv: false,
        }
    }

    pub fn prefix(prefix: &[u8]) -> Self
    {
        let key = prefix.to_vec();
        let end_key = {
            let mut end = key.clone();

            for i in (0..end.len()).rev() {
                if end[i] < 0xff {
                    end[i] += 1;
                    end = end[0..=i].to_vec();
                    break;
                }
            }

            end
        };
        Self {
            key,
            end_key: Some(end_key),
            prev_kv: false,
        }
    }

    pub fn with_prev_kv(mut self) -> Self {
        self.prev_kv = true;
        self
    }
}

impl Into<rpc::DeleteRangeRequest> for DeleteRequest {
    fn into(self) -> rpc::DeleteRangeRequest {
        let mut req = rpc::DeleteRangeRequest::new();

        req.set_key(self.key);
        req.set_prev_kv(self.prev_kv);
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }
        req
    }
}

#[derive(Debug)]
pub struct DeleteResponse {
    resp: rpc::DeleteRangeResponse,
}

impl DeleteResponse {
    pub fn prev_kvs(&self) -> Vec<KeyValue> {
        // FIXME perf
        self.resp
            .get_prev_kvs()
            .iter()
            .map(|kv| From::from(kv.clone()))
            .collect()
    }

    pub fn deleted(&self) -> i64 {
        self.resp.get_deleted()
    }

    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }
}

impl From<rpc::DeleteRangeResponse> for DeleteResponse {
    fn from(resp: rpc::DeleteRangeResponse) -> Self {
        Self { resp }
    }
}
