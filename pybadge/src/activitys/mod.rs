pub(crate) mod card_selecter;
pub(crate) mod game_over;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Activity {
	Selecter,
	Waiting,
	GameOver(m3_models::GameOver)
}
