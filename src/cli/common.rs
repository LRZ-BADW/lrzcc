pub(crate) trait Execute {
    fn execute(
        &self,
        api: lrzcc::Api,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
