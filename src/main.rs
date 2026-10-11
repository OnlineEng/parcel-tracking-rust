// enum means the parcel can be in only one state at a time
enum ParcelStatus {
    Preparing,
    InTransit { courier: String },
    Delivered { recipient: String },
    Returned { reason: String },
}

// Takes ownership, match must cover every variant
// format! builds a String, much tidier than push_str - found in the latter .rs materials
fn status_message(status: ParcelStatus) -> String {
    match status {
        ParcelStatus::Preparing => String::from("Parcel is being prepared."),
        ParcelStatus::InTransit { courier } => format!("Parcel is with courier: {courier}."),
        ParcelStatus::Delivered { recipient } => format!("Parcel was delivered to {recipient}."),
        ParcelStatus::Returned { reason } => format!("Parcel was returned: {reason}."),
    }
}

fn print_delivery_note(note: Option<&str>) {
    if let Some(text) = note {
        println!("Delivery note: {text}");
    } else {
        println!("There is no delivery note.");
    }
}

// Borrows only (&), caller can still use the parcel afterward
fn announce_delivery(status: &ParcelStatus) {
    if let ParcelStatus::Delivered { recipient } = status {
        println!("Parcel was delivered to {recipient}.");
    } else {
        println!("Parcel has not been delivered yet.");
    }
}

// Takes ownership, else must leave the function
fn create_receipt(status: ParcelStatus) {
    let ParcelStatus::Delivered { recipient } = status else {
        println!("Cannot create a receipt: the parcel was not delivered.");
        return;
    };
    println!("Receipt created for {recipient}.");
}

fn main() {
    let preparing = ParcelStatus::Preparing;
    let in_transit = ParcelStatus::InTransit { courier: String::from("Aino") };
    let delivered = ParcelStatus::Delivered { recipient: String::from("Mikko") };
    let returned = ParcelStatus::Returned { reason: String::from("address not found") };

    // Value gets moved so they can't be used again
    println!("Status: {}", status_message(preparing));
    println!("Status: {}", status_message(in_transit));
    println!("Status: {}", status_message(delivered));
    println!("Status: {}", status_message(returned));

    print_delivery_note(Some("Leave the parcel at reception."));
    print_delivery_note(None);

    // New values
    let delivered_parcel = ParcelStatus::Delivered { recipient: String::from("Mikko") };
    let pending_parcel = ParcelStatus::Preparing;

    announce_delivery(&delivered_parcel);
    announce_delivery(&pending_parcel);

    // These move the parcels so they have to come after the borrows above
    create_receipt(delivered_parcel);
    create_receipt(pending_parcel);
}
