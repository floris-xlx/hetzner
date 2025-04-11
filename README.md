# Hetzner DNS API Rust SDK

### Hetzner API Coverage

#### `zones`
- [x] **GetAllZones**: Retrieve a list of all DNS zones associated with your account.
- [ ] **CreateZone**: Create a new DNS zone.
- [ ] **GetZone**: Retrieve detailed information about a specific DNS zone.
- [ ] **UpdateZone**: Update the settings of an existing DNS zone.
- [ ] **DeleteZone**: Delete a DNS zone.
- [ ] **ImportZoneFilePlain**: Import a DNS zone file in plain text format.
- [ ] **ExportZoneFile**: Export a DNS zone file.
- [ ] **ValidateZoneFilePlain**: Validate a DNS zone file in plain text format.

#### `records`
- [x] **GetAllRecords**: Retrieve a list of all DNS records within a zone.
- [x] **CreateRecord**: Create a new DNS record within a zone.
- [x] **GetRecord**: Retrieve detailed information about a specific DNS record.
- [x] **UpdateRecord**: Update an existing DNS record.
- [x] **DeleteRecord**: Delete a DNS record.
- [ ] **BulkCreateRecords**: Create multiple DNS records in bulk.
- [ ] **BulkUpdateRecords**: Update multiple DNS records in bulk.

#### `primary servers`
- [ ] **GetAllPrimaryServers**: Retrieve a list of all primary DNS servers.
- [ ] **CreatePrimaryServer**: Create a new primary DNS server.
- [ ] **GetPrimaryServer**: Retrieve detailed information about a specific primary DNS server.
- [ ] **UpdatePrimaryServer**: Update the settings of an existing primary DNS server.
- [ ] **DeletePrimaryServer**: Delete a primary DNS server.
