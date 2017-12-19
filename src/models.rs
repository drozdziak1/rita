use super::schema::neighbors;
use ip_network::IpNetwork;

#[derive(Queryable)]
pub struct Neighbor {
  pub id: i32,
  pub debt: u64,
  pub ip: IpNetwork,
}

#[derive(Insertable)]
#[table_name = "neighbors"]
pub struct NewNeighbor<'a> {
  pub debt: &'a u64,
  pub ip: &'a IpNetwork,
}
