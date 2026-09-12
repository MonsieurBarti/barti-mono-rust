# One OpenAPI document or per-cell specs merged at app?

Type: grilling
Label: wayfinder:grilling
Blocked by: 09

## Question

Does each cell emit a fragment that `app` merges, or does the composition root own one document?

Public REST is nested in `app/http.rs`. Cell name is not a path segment. Collision is a review reject.
