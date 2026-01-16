use axum::{
    extract::{Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use rand::Rng;
use sea_orm::EntityTrait;
use serde::Deserialize;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::entities::warehouse;
use crate::utils::error::AppError;

#[derive(Debug, Deserialize, Validate)]
pub struct GenerateSampleRequest {
    #[validate(range(min = 1, message = "warehouse_id must be a positive integer"))]
    pub warehouse_id: i32,

    #[validate(range(
        min = 1,
        max = 10_000_000,
        message = "count must be between 1 and 1,000,000"
    ))]
    pub count: Option<i32>,
}

const ITEM_NAMES: &[&str] = &[
    "Widget",
    "Gadget",
    "Component",
    "Assembly",
    "Module",
    "Unit",
    "Part",
    "Device",
    "Element",
    "Piece",
    "Section",
    "Block",
    "Panel",
    "Frame",
    "Bracket",
    "Connector",
    "Adapter",
    "Housing",
    "Cover",
    "Shield",
    "Plate",
    "Bar",
    "Rod",
    "Tube",
    "Fastener",
    "Bolt",
    "Nut",
    "Screw",
    "Washer",
    "Rivet",
    "Pin",
    "Clip",
    "Bearing",
    "Gear",
    "Shaft",
    "Pulley",
    "Belt",
    "Chain",
    "Spring",
    "Valve",
    "Pump",
    "Motor",
    "Sensor",
    "Switch",
    "Relay",
    "Fuse",
    "Capacitor",
    "Resistor",
];

const ITEM_ADJECTIVES: &[&str] = &[
    "Standard",
    "Premium",
    "Heavy-Duty",
    "Compact",
    "Industrial",
    "Commercial",
    "Professional",
    "Economy",
    "Deluxe",
    "Ultra",
    "Mini",
    "Mega",
    "Super",
    "Pro",
    "Basic",
    "Advanced",
    "Enhanced",
    "Reinforced",
    "Lightweight",
    "Durable",
];

const UNITS_OF_MEASURE: &[&str] = &["EA", "PK", "CS", "BX", "PL", "KG", "LB", "M", "FT"];

pub async fn generate_sample(
    State(db): State<DatabaseConnection>,
    Query(params): Query<GenerateSampleRequest>,
) -> Result<impl IntoResponse, AppError> {
    params.validate().map_err(|e| {
        AppError::validation(crate::utils::error::AppError::collect_validation_errors(&e))
    })?;

    let warehouse_id = params.warehouse_id;
    let count = params.count.unwrap_or(100);

    let warehouse = warehouse::Entity::find_by_id(warehouse_id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouse: {}", e)))?;

    if warehouse.is_none() {
        return Err(AppError::not_found(format!(
            "Warehouse with id {} not found",
            warehouse_id
        )));
    }

    let mut csv_content = String::new();
    csv_content.push_str("sku,location_code,quantity,name,unit_of_measure,warehouse_id\n");

    let mut rng = rand::thread_rng();

    for i in 1..=count {
        let sku = format!("SKU-{:06}", i);
        let location_code = generate_location_code(&mut rng);
        let quantity = rng.gen_range(1..=1000);
        let name = generate_item_name(&mut rng);
        let unit_of_measure = UNITS_OF_MEASURE[rng.gen_range(0..UNITS_OF_MEASURE.len())];

        csv_content.push_str(&format!(
            "{},{},{},{},{},{}\n",
            sku, location_code, quantity, name, unit_of_measure, warehouse_id
        ));
    }

    let headers = [
        (header::CONTENT_TYPE, "text/csv; charset=utf-8".to_string()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"sample_receipt_{}rows.csv\"", count),
        ),
    ];

    Ok((StatusCode::OK, headers, csv_content))
}

fn generate_location_code<R: Rng>(rng: &mut R) -> String {
    let aisle = (b'A' + rng.gen_range(0..26)) as char;
    let bay = rng.gen_range(1..=50);
    let level = rng.gen_range(1..=5);
    let position = rng.gen_range(1..=10);

    format!("{}{:02}-{}-{:02}", aisle, bay, level, position)
}

fn generate_item_name<R: Rng>(rng: &mut R) -> String {
    let adjective = ITEM_ADJECTIVES[rng.gen_range(0..ITEM_ADJECTIVES.len())];
    let name = ITEM_NAMES[rng.gen_range(0..ITEM_NAMES.len())];
    let variant = rng.gen_range(100..=999);

    format!("{} {} {}", adjective, name, variant)
}
