




pub(crate) mod card_selecter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Activity {
	Selecter,
	Waiting
}
