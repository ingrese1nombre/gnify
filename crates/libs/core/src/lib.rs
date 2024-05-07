pub mod user;
pub mod role;
pub mod device;

gnify::text! {
    Privilege => 
        pattern: r"^([A-Z]+\s)*[A-Z]+$";
        min: 4;
        max: 32;
}