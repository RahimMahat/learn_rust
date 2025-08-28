use pretty_assertions;

use testing::attractions::Museum;
use testing::management::VenueManagement;

#[test]
fn veneue_management_interacts_with_venue() {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");

    let mut veneu_mgmt = VenueManagement::new(museum);
    veneu_mgmt.make_money();
    assert_eq!(veneu_mgmt.venue.revenue, 25);
}
