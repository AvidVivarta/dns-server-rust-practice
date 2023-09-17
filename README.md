# [DNS Server in Rust](https://github.com/EmilHernvall/dnsguide/blob/master/README.md)

- DNS packets are sent using UDP transpot and are limited to 512 bytes.
- But can be used over TCP and using eDNS we can extend the packet size.
- Queries and response use the same format
- Structure of DNS packet:
  | Section | Size | Type | Purpose|
  |------|--------|--------|---------|
  | Header| 12 Bytes | Header | Information about the query/response |
  | Question Section | Variable | List of Questions | In practice only a single question indicating the query name (domain) and the record type of interest |
  | Anser Section | Variable | List of Records | The relevant records of the requested type. |
  | Authority Section | Variable | List of Records | An list of name server (NS records), used for resolving queries recursively. |
  | Additional Section | Variable | List of Records | Additional records, that might be useful. For instance, the corresponding A records for NS records. |

* Header Structure: 

| RFC Name | Descriptive Name     | Length             | Description                                                                                                                                                                         |
| -------- | -------------------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ID       | Packet Identifier    | 16 bits            | A random identifier is assigned to query packets. Response packets must reply with the same id. This is needed to differentiate responses due to the stateless nature of UDP.       |
| QR       | Query Response       | 1 bit              | 0 for queries, 1 for responses.                                                                                                                                                     |
| OPCODE   | Operation Code       | 4 bits             | Typically always 0, see RFC1035 for details.                                                                                                                                        |
| AA       | Authoritative Answer | 1 bit              | Set to 1 if the responding server is authoritative - that is, it "owns" - the domain queried.                                                                                       |
| TC       | Truncated Message    | 1 bit              | Set to 1 if the message length exceeds 512 bytes. Traditionally a hint that the query can be reissued using TCP, for which the length limitation doesn't apply.                     |
| RD       | Recursion Desired    | 1 bit              | Set by the sender of the request if the server should attempt to resolve the query recursively if it does not have an answer readily available.                                     |
| RA       | Recursion Available  | 1 bit              | Set by the server to indicate whether or not recursive queries are allowed.                                                                                                         |
| Z        | Reserved             | 3 bits             | Originally reserved for later use, but now used for DNSSEC queries.                                                                                                                 |
| RCODE    | Response Code        | 4 bits             | Set by the server to indicate the status of the response, i.e. whether or not it was successful or failed, and in the latter case providing details about the cause of the failure. |
| QDCOUNT  | Question Count       | 16 bits            | The number of entries in the Question Section                                                                                                                                       |
| ANCOUNT  | Answer Count         | 16 bits            | The number of entries in the Answer Section                                                                                                                                         |
| NSCOUNT  | Authority Count      | 16 bits            | The number of entries in the Authority Section                                                                                                                                      |
| ARCOUNT  | Additional Count     | 16 bits            | The number of entries in the Additional Section                                                                                                                                     |

