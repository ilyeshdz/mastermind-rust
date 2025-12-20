#[derive(Debug)]
pub enum RulesError {
    InvalidCodeLen,
    InvalidLimit,
    InvalidAvailableSymbols,
}

pub type CodeLen = u8;

#[derive(Debug)]
pub enum Limit {
    Attempts { count: u16 },
    Time { seconds: u32 },
    NoLimitation,
}

#[derive(Debug)]
pub struct Rules {
    code_len: CodeLen,
    limit: Limit,
    available_symbols: u8,
}

impl Rules {
    pub fn new(code_len: CodeLen, limit: Limit, available_symbols: u8) -> Result<Self, RulesError> {
        if code_len == 0 {
            return Err(RulesError::InvalidCodeLen);
        }

        if available_symbols == 0 {
            return Err(RulesError::InvalidAvailableSymbols);
        }

        match limit {
            Limit::Attempts { count } if count == 0 => return Err(RulesError::InvalidLimit),
            Limit::Time { seconds } if seconds == 0 => return Err(RulesError::InvalidLimit),
            _ => {}
        }

        Ok(Rules {
            code_len,
            limit,
            available_symbols,
        })
    }

    pub fn code_len(&self) -> CodeLen {
        self.code_len
    }

    pub fn limit(&self) -> &Limit {
        &self.limit
    }

    pub fn available_symbols(&self) -> u8 {
        self.available_symbols
    }
}
