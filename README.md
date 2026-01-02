# Data Scraping – High-Level Overview

## Purpose

This project is responsible for collecting publicly available data from approved third-party websites and transforming it into a structured format suitable for storage, analysis, or downstream processing.

The system is designed to be:
- Permission-based
- Predictable
- Maintainable
- Easy to migrate to an official API when available

---

## What “Data Scraping” Means Here

In this project, data scraping refers to:

- Fetching public web pages via standard HTTP requests
- Parsing server-rendered HTML responses
- Extracting specific, well-defined data elements
- Normalizing and storing the extracted data

This project explicitly **does not** involve:
- Packet sniffing or traffic interception
- TLS breaking or MITM techniques
- CAPTCHA bypassing
- Credential abuse
- Aggressive or recursive crawling

---

## High-Level Architecture


Each stage is isolated to ensure the system remains modular and replaceable.

---

## Core Components

### Fetcher
- Sends HTTP GET requests to predefined URLs
- Uses a fixed, identifiable User-Agent
- Applies rate limits and request delays
- Handles network timeouts and transient failures

### Parser
- Parses HTML documents
- Extracts only required elements (e.g. item links, metadata)
- Avoids fragile or deeply nested selectors

### Normalizer
- Converts raw data into stable internal models
- Deduplicates records
- Normalizes URLs, identifiers, and formats

### Storage / Output
- Storage layer is decoupled from scraping logic
- Can be extended to support:
  - Databases
  - CSV / JSON / Parquet files
  - Message queues or streams

---

## Scope Control

The scraper operates within a clearly defined scope:
- Explicit page ranges
- Known URL patterns
- No recursive traversal
- No infinite pagination

This prevents:
- Excessive load on the source system
- Uncontrolled data growth
- Operational or legal risks

---

## Configuration

All runtime parameters are externalized via environment variables:
- Base URLs
- Page ranges
- Request delays

This allows configuration changes without modifying code and supports multiple environments.

---

## Update & Change Handling

The system assumes that:
- Data may change over time
- Items may be updated or removed
- The source structure may evolve

Instead of hard deletes:
- Items are tracked using a “last seen” approach
- Removals are detected over time
- Changes can be diffed and analyzed

This strategy improves reliability and historical tracking.

---

## Legal & Ethical Considerations

This project operates under:
- Explicit approval from the data source
- Respect for server stability and capacity
- Conservative request rates
- No circumvention of access controls

If an official API becomes available, scraping should be reduced or replaced accordingly.

---

## When to Use Scraping

Scraping is used as a **bridge solution** when:
- No official API exists
- Data access is approved
- Update frequency is moderate

Scraping is not intended as a permanent substitute for APIs.

---

## Future Improvements

Possible future enhancements include:
- Migration to API-based ingestion
- Change detection and diff tracking
- Scheduling and automation
- Metrics and monitoring
- Controlled concurrency and backpressure

---

## Summary

This data scraping system is:
- Purpose-built
- Permission-based
- Minimal in scope
- Modular by design
- Ready for future API migration

The primary goal is **data quality and system stability**, not aggressive crawling.
