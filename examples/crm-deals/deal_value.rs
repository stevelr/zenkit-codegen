/// This is a sample program using a zenkit client automatically generated
/// from the sample template "CRM For Sales" using https://github.com/stevelr/zenkit-codegen
///
/// This program prints the "expected deal value",
/// a probability-weighted sum of pending orders for each company.
/// See README.md for build instructions
use crm_for_sales::{DealTrackerList, Error};

/// Print deals in progress with probability-weighted value
async fn get_deal_value() -> Result<(), Error> {
    // Initialize zenkit client with api token from environment
    crm_for_sales::initialize_zenkit_api(None, None)?;

    let mut total = 0.0;
    println!("Deals in progress:");
    // iterate through all deals ...
    for deal in DealTrackerList::get_items().await?.iter() {
        // filter for deals where Stage is "In progress"
        if deal.is_stage_in_progress() {
            let title = deal.get_title().unwrap_or("untitled");
            // Probability ranges from 0-100 (a percentage). Use default of 100 if empty.
            let prob = deal.get_deal_probability_in().unwrap_or(100) as f64;
            // Multiply "Order Total" formula by probability
            let weighted = match deal.get_order_total() {
                Some(val) => {
                    let weighted_value = prob * val / 100.0;
                    total += weighted_value;
                    format!("{:.2}", weighted_value)
                }
                None => "(incomplete)".to_string(), // print this if order total isn't calculated
            };
            println!("{:55} {}", title, weighted);
        }
    }
    println!("{:55} {:.2}", "Total weighted value", total);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    if let Err(e) = get_deal_value().await {
        eprintln!("Error: {:#?}", e);
    }

    Ok(())
}
