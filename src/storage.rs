pub trait Storable<T> {
    fn new(role: &String, content: &String) -> T;
    // insert a new record
    fn create(&self, conn: &rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>>;
    // return all records
    fn all(conn: &rusqlite::Connection) -> Result<Vec<T>, Box<dyn std::error::Error>>;
    // Initialize the table in the database
    fn init(conn: &rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>>;
}
