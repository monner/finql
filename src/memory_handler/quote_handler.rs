use super::InMemoryDB;
use crate::currency::Currency;
use crate::data_handler::{DataError, QuoteHandler};
use crate::quote::{MarketDataSource, Quote, Ticker};
use chrono::{DateTime, Utc, MIN_DATE};

/// Handler for globally available market data quotes information
impl QuoteHandler for InMemoryDB {
    // insert, get, update and delete for market data sources
    fn insert_md_source(&mut self, source: &MarketDataSource) -> Result<usize, DataError> {
        self.md_sources.insert(source)
    }

    fn get_md_source_by_id(&mut self, id: usize) -> Result<MarketDataSource, DataError> {
        self.md_sources.get_by_id(id)
    }

    fn get_all_md_sources(&mut self) -> Result<Vec<MarketDataSource>, DataError> {
        self.md_sources.get_all()
    }

    fn update_md_source(&mut self, source: &MarketDataSource) -> Result<(), DataError> {
        self.md_sources.update(source)
    }

    fn delete_md_source(&mut self, id: usize) -> Result<(), DataError> {
        self.md_sources.delete(id)
    }

    // insert, get, update and delete for market data ticker
    fn insert_ticker(&mut self, asset: &Ticker) -> Result<usize, DataError> {
        self.ticker_map.insert(asset)
    }

    fn get_ticker_by_id(&mut self, id: usize) -> Result<Ticker, DataError> {
        self.ticker_map.get_by_id(id)
    }

    fn get_all_ticker_for_source(&mut self, source_id: usize) -> Result<Vec<Ticker>, DataError> {
        let mut all_ticker = Vec::new();
        for ticker in self.ticker_map.items.values() {
            if ticker.source == source_id {
                all_ticker.push(ticker.clone())
            }
        }
        Ok(all_ticker)
    }

    fn update_ticker(&mut self, asset: &Ticker) -> Result<(), DataError> {
        self.ticker_map.update(asset)
    }

    fn delete_ticker(&mut self, id: usize) -> Result<(), DataError> {
        self.ticker_map.delete(id)
    }

    // insert, get, update and delete for quotes
    fn insert_quote(&mut self, quote: &Quote) -> Result<usize, DataError> {
        self.quotes.insert(quote)
    }

    fn get_last_quote_before(
        &mut self,
        ticker_id: usize,
        time: DateTime<Utc>,
    ) -> Result<(Quote, Currency), DataError> {
        let mut last_quote = Quote {
            id: None,
            ticker: 0,
            price: 0.0,
            time: MIN_DATE.and_hms(0, 0, 0),
            volume: Some(0.0),
        };
        // For now, use very inefficient linear search
        for quote in self.quotes.items.values() {
            if quote.ticker == ticker_id {
                if quote.time <= time {
                    if last_quote.id == None {
                        last_quote = quote.clone()
                    } else if last_quote.time < quote.time {
                        last_quote = quote.clone()
                    }
                }
            }
        }
        if last_quote.id == None {
            return Err(DataError::NotFound(
                "No valid quote found before specified date".to_string(),
            ));
        }
        let ticker = self.get_ticker_by_id(ticker_id)?;
        Ok((last_quote, ticker.currency))
    }

    fn get_all_quotes_for_ticker(&mut self, ticker_id: usize) -> Result<Vec<Quote>, DataError> {
        let mut all_quotes = Vec::new();
        for quote in self.quotes.items.values() {
            if quote.ticker == ticker_id {
                all_quotes.push(quote.clone())
            }
        }
        Ok(all_quotes)
    }

    fn update_quote(&mut self, quote: &Quote) -> Result<(), DataError> {
        self.quotes.update(quote)
    }

    fn delete_quote(&mut self, id: usize) -> Result<(), DataError> {
        self.quotes.delete(id)
    }
}
