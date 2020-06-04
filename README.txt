// install cli without mysql client
cargo install diesel_cli --no-default-features --features postgres
// initial migration flows
diesel setup
// create migration script
diesel migration generate XXXXXXXX
// run migrations
diesel migration run
// rollback migration
diesel migration redo
