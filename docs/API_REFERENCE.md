# API Reference

This reference is generated from `openapi.json`.

## Servers API

Total operations: **33**

| Method | Path | Operation ID | Summary |
| --- | --- | --- | --- |
| `GET` | `/servers` | `list_servers` | List Servers |
| `POST` | `/servers` | `create_server` | Create a Server |
| `DELETE` | `/servers/{id}` | `delete_server` | Delete a Server |
| `GET` | `/servers/{id}` | `get_server` | Get a Server |
| `PUT` | `/servers/{id}` | `update_server` | Update a Server |
| `GET` | `/servers/{id}/actions` | `list_server_actions` | List Actions for a Server |
| `GET` | `/servers/{id}/actions/{action_id}` | `get_server_action` | Get an Action for a Server |
| `POST` | `/servers/{id}/actions/add_to_placement_group` | `add_server_to_placement_group` | Add a Server to a Placement Group |
| `POST` | `/servers/{id}/actions/attach_iso` | `attach_server_iso` | Attach an ISO to a Server |
| `POST` | `/servers/{id}/actions/attach_to_network` | `attach_server_to_network` | Attach a Server to a Network |
| `POST` | `/servers/{id}/actions/change_alias_ips` | `change_server_alias_ips` | Change alias IPs of a Network |
| `POST` | `/servers/{id}/actions/change_dns_ptr` | `change_server_dns_ptr` | Change reverse DNS entry for this Server |
| `POST` | `/servers/{id}/actions/change_protection` | `change_server_protection` | Change Server Protection |
| `POST` | `/servers/{id}/actions/change_type` | `change_server_type` | Change the Type of a Server |
| `POST` | `/servers/{id}/actions/create_image` | `create_server_image` | Create Image from a Server |
| `POST` | `/servers/{id}/actions/detach_from_network` | `detach_server_from_network` | Detach a Server from a Network |
| `POST` | `/servers/{id}/actions/detach_iso` | `detach_server_iso` | Detach an ISO from a Server |
| `POST` | `/servers/{id}/actions/disable_backup` | `disable_server_backup` | Disable Backups for a Server |
| `POST` | `/servers/{id}/actions/disable_rescue` | `disable_server_rescue` | Disable Rescue Mode for a Server |
| `POST` | `/servers/{id}/actions/enable_backup` | `enable_server_backup` | Enable and Configure Backups for a Server |
| `POST` | `/servers/{id}/actions/enable_rescue` | `enable_server_rescue` | Enable Rescue Mode for a Server |
| `POST` | `/servers/{id}/actions/poweroff` | `poweroff_server` | Power off a Server |
| `POST` | `/servers/{id}/actions/poweron` | `poweron_server` | Power on a Server |
| `POST` | `/servers/{id}/actions/reboot` | `reboot_server` | Soft-reboot a Server |
| `POST` | `/servers/{id}/actions/rebuild` | `rebuild_server` | Rebuild a Server from an Image |
| `POST` | `/servers/{id}/actions/remove_from_placement_group` | `remove_server_from_placement_group` | Remove from Placement Group |
| `POST` | `/servers/{id}/actions/request_console` | `request_server_console` | Request Console for a Server |
| `POST` | `/servers/{id}/actions/reset` | `reset_server` | Reset a Server |
| `POST` | `/servers/{id}/actions/reset_password` | `reset_server_password` | Reset root Password of a Server |
| `POST` | `/servers/{id}/actions/shutdown` | `shutdown_server` | Shutdown a Server |
| `GET` | `/servers/{id}/metrics` | `get_server_metrics` | Get Metrics for a Server |
| `GET` | `/servers/actions` | `list_servers_actions` | List Actions |
| `GET` | `/servers/actions/{id}` | `get_servers_action` | Get an Action |

## Domain and DNS API

Total operations: **25**

| Method | Path | Operation ID | Summary |
| --- | --- | --- | --- |
| `GET` | `/zones` | `list_zones` | List Zones |
| `POST` | `/zones` | `create_zone` | Create a Zone |
| `DELETE` | `/zones/{id_or_name}` | `delete_zone` | Delete a Zone |
| `GET` | `/zones/{id_or_name}` | `get_zone` | Get a Zone |
| `PUT` | `/zones/{id_or_name}` | `update_zone` | Update a Zone |
| `GET` | `/zones/{id_or_name}/actions` | `list_zone_actions` | List Actions for a Zone |
| `GET` | `/zones/{id_or_name}/actions/{action_id}` | `get_zone_action` | Get an Action for a Zone |
| `POST` | `/zones/{id_or_name}/actions/change_primary_nameservers` | `change_zone_primary_nameservers` | Change a Zone's Primary Nameservers |
| `POST` | `/zones/{id_or_name}/actions/change_protection` | `change_zone_protection` | Change a Zone's Protection |
| `POST` | `/zones/{id_or_name}/actions/change_ttl` | `change_zone_ttl` | Change a Zone's Default TTL |
| `POST` | `/zones/{id_or_name}/actions/import_zonefile` | `import_zone_zonefile` | Import a Zone file |
| `GET` | `/zones/{id_or_name}/rrsets` | `list_zone_rrsets` | List RRSets |
| `POST` | `/zones/{id_or_name}/rrsets` | `create_zone_rrset` | Create an RRSet |
| `DELETE` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}` | `delete_zone_rrset` | Delete an RRSet |
| `GET` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}` | `get_zone_rrset` | Get an RRSet |
| `PUT` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}` | `update_zone_rrset` | Update an RRSet |
| `POST` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}/actions/add_records` | `add_zone_rrset_records` | Add Records to an RRSet |
| `POST` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}/actions/change_protection` | `change_zone_rrset_protection` | Change an RRSet's Protection |
| `POST` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}/actions/change_ttl` | `change_zone_rrset_ttl` | Change an RRSet's TTL |
| `POST` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}/actions/remove_records` | `remove_zone_rrset_records` | Remove Records from an RRSet |
| `POST` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}/actions/set_records` | `set_zone_rrset_records` | Set Records of an RRSet |
| `POST` | `/zones/{id_or_name}/rrsets/{rr_name}/{rr_type}/actions/update_records` | `update_zone_rrset_records` | Update Records of an RRSet |
| `GET` | `/zones/{id_or_name}/zonefile` | `get_zone_zonefile` | Export a Zone file |
| `GET` | `/zones/actions` | `list_zones_actions` | List Actions |
| `GET` | `/zones/actions/{id}` | `get_zones_action` | Get an Action |

## Private Network API

Total operations: **15**

| Method | Path | Operation ID | Summary |
| --- | --- | --- | --- |
| `GET` | `/networks` | `list_networks` | List Networks |
| `POST` | `/networks` | `create_network` | Create a Network |
| `DELETE` | `/networks/{id}` | `delete_network` | Delete a Network |
| `GET` | `/networks/{id}` | `get_network` | Get a Network |
| `PUT` | `/networks/{id}` | `update_network` | Update a Network |
| `GET` | `/networks/{id}/actions` | `list_network_actions` | List Actions for a Network |
| `GET` | `/networks/{id}/actions/{action_id}` | `get_network_action` | Get an Action for a Network |
| `POST` | `/networks/{id}/actions/add_route` | `add_network_route` | Add a route to a Network |
| `POST` | `/networks/{id}/actions/add_subnet` | `add_network_subnet` | Add a subnet to a Network |
| `POST` | `/networks/{id}/actions/change_ip_range` | `change_network_ip_range` | Change IP range of a Network |
| `POST` | `/networks/{id}/actions/change_protection` | `change_network_protection` | Change Network Protection |
| `POST` | `/networks/{id}/actions/delete_route` | `delete_network_route` | Delete a route from a Network |
| `POST` | `/networks/{id}/actions/delete_subnet` | `delete_network_subnet` | Delete a subnet from a Network |
| `GET` | `/networks/actions` | `list_networks_actions` | List Actions |
| `GET` | `/networks/actions/{id}` | `get_networks_action` | Get an Action |

## Load Balancer API

Total operations: **25**

| Method | Path | Operation ID | Summary |
| --- | --- | --- | --- |
| `GET` | `/load_balancer_types` | `list_load_balancer_types` | List Load Balancer Types |
| `GET` | `/load_balancer_types/{id}` | `get_load_balancer_type` | Get a Load Balancer Type |
| `GET` | `/load_balancers` | `list_load_balancers` | List Load Balancers |
| `POST` | `/load_balancers` | `create_load_balancer` | Create a Load Balancer |
| `DELETE` | `/load_balancers/{id}` | `delete_load_balancer` | Delete a Load Balancer |
| `GET` | `/load_balancers/{id}` | `get_load_balancer` | Get a Load Balancer |
| `PUT` | `/load_balancers/{id}` | `update_load_balancer` | Update a Load Balancer |
| `GET` | `/load_balancers/{id}/actions` | `list_load_balancer_actions` | List Actions for a Load Balancer |
| `GET` | `/load_balancers/{id}/actions/{action_id}` | `get_load_balancer_action` | Get an Action for a Load Balancer |
| `POST` | `/load_balancers/{id}/actions/add_service` | `add_load_balancer_service` | Add Service |
| `POST` | `/load_balancers/{id}/actions/add_target` | `add_load_balancer_target` | Add Target |
| `POST` | `/load_balancers/{id}/actions/attach_to_network` | `attach_load_balancer_to_network` | Attach a Load Balancer to a Network |
| `POST` | `/load_balancers/{id}/actions/change_algorithm` | `change_load_balancer_algorithm` | Change Algorithm |
| `POST` | `/load_balancers/{id}/actions/change_dns_ptr` | `change_load_balancer_dns_ptr` | Change reverse DNS entry for this Load Balancer |
| `POST` | `/load_balancers/{id}/actions/change_protection` | `change_load_balancer_protection` | Change Load Balancer Protection |
| `POST` | `/load_balancers/{id}/actions/change_type` | `change_load_balancer_type` | Change the Type of a Load Balancer |
| `POST` | `/load_balancers/{id}/actions/delete_service` | `delete_load_balancer_service` | Delete Service |
| `POST` | `/load_balancers/{id}/actions/detach_from_network` | `detach_load_balancer_from_network` | Detach a Load Balancer from a Network |
| `POST` | `/load_balancers/{id}/actions/disable_public_interface` | `disable_load_balancer_public_interface` | Disable the public interface of a Load Balancer |
| `POST` | `/load_balancers/{id}/actions/enable_public_interface` | `enable_load_balancer_public_interface` | Enable the public interface of a Load Balancer |
| `POST` | `/load_balancers/{id}/actions/remove_target` | `remove_load_balancer_target` | Remove Target |
| `POST` | `/load_balancers/{id}/actions/update_service` | `update_load_balancer_service` | Update Service |
| `GET` | `/load_balancers/{id}/metrics` | `get_load_balancer_metrics` | Get Metrics for a LoadBalancer |
| `GET` | `/load_balancers/actions` | `list_load_balancers_actions` | List Actions |
| `GET` | `/load_balancers/actions/{id}` | `get_load_balancers_action` | Get an Action |

## Storage API

Total operations: **24**

| Method | Path | Operation ID | Summary |
| --- | --- | --- | --- |
| `GET` | `/images` | `list_images` | List Images |
| `DELETE` | `/images/{id}` | `delete_image` | Delete an Image |
| `GET` | `/images/{id}` | `get_image` | Get an Image |
| `PUT` | `/images/{id}` | `update_image` | Update an Image |
| `GET` | `/images/{id}/actions` | `list_image_actions` | List Actions for an Image |
| `GET` | `/images/{id}/actions/{action_id}` | `get_image_action` | Get an Action for an Image |
| `POST` | `/images/{id}/actions/change_protection` | `change_image_protection` | Change Image Protection |
| `GET` | `/images/actions` | `list_images_actions` | List Actions |
| `GET` | `/images/actions/{id}` | `get_images_action` | Get an Action |
| `GET` | `/isos` | `list_isos` | List ISOs |
| `GET` | `/isos/{id}` | `get_iso` | Get an ISO |
| `GET` | `/volumes` | `list_volumes` | List Volumes |
| `POST` | `/volumes` | `create_volume` | Create a Volume |
| `DELETE` | `/volumes/{id}` | `delete_volume` | Delete a Volume |
| `GET` | `/volumes/{id}` | `get_volume` | Get a Volume |
| `PUT` | `/volumes/{id}` | `update_volume` | Update a Volume |
| `GET` | `/volumes/{id}/actions` | `list_volume_actions` | List Actions for a Volume |
| `GET` | `/volumes/{id}/actions/{action_id}` | `get_volume_action` | Get an Action for a Volume |
| `POST` | `/volumes/{id}/actions/attach` | `attach_volume` | Attach Volume to a Server |
| `POST` | `/volumes/{id}/actions/change_protection` | `change_volume_protection` | Change Volume Protection |
| `POST` | `/volumes/{id}/actions/detach` | `detach_volume` | Detach Volume |
| `POST` | `/volumes/{id}/actions/resize` | `resize_volume` | Resize Volume |
| `GET` | `/volumes/actions` | `list_volumes_actions` | List Actions |
| `GET` | `/volumes/actions/{id}` | `get_volumes_action` | Get an Action |

## Actions API

Total operations: **2**

| Method | Path | Operation ID | Summary |
| --- | --- | --- | --- |
| `GET` | `/actions` | `get_actions` | Get multiple Actions |
| `GET` | `/actions/{id}` | `get_action` | Get an Action |

