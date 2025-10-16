# Reimburser

A Rust-based CLI tool for automated expense reimbursement calculation from Dutch public transport invoices (NS and GVB). The tool parses PDF invoices and filters work-related trips to calculate reimbursable amounts.

## Features

- **PDF Invoice Parsing**: Automatically extracts trip data from NS (Nederlandse Spoorwegen) PDF invoices
- **Multi-Provider Support**: Handles both NS train and GVB (Amsterdam public transport) trips
- **Smart Trip Filtering**: 
  - Filters trips by specified departure and arrival stations
  - Automatically detects multi-leg journeys (transfers)
  - Supports bidirectional commuting (home→work and work→home)
- **Workday Filtering**: Only includes trips on workdays (Monday-Friday)
- **Detailed Reporting**: Generates formatted tables with trip details and calculates subtotals per provider

## Architecture

The application follows a modular architecture with clear separation of concerns:

```
src/
├── main.rs           # CLI entry point and argument parsing
├── data.rs           # Core data structures and station databases
├── ns_pdf_scanner.rs # PDF parsing and text extraction logic
└── trip_filter.rs    # Business logic for filtering trips
```

### Core Components

1. **Data Layer** (`data.rs`):
   - `Trip` struct: Represents a single journey with date, provider, from/to stations, and price
   - `Provider` enum: Distinguishes between NS and GVB trips
   - Complete station databases for both NS (399 stations) and GVB (543 stations)

2. **PDF Scanner** (`ns_pdf_scanner.rs`):
   - Uses regex patterns to extract trip data from PDF text
   - Handles different formats for NS and GVB invoices
   - Parses dates, stations, and prices from invoice lines

3. **Trip Filtering** (`trip_filter.rs`):
   - **Station Filter**: Identifies work-related trips based on specified stations
   - **Chain Detection**: Automatically groups multi-leg journeys on the same day
   - **Workday Filter**: Excludes weekend trips

4. **CLI Interface** (`main.rs`):
   - Uses `clap` for argument parsing
   - Generates formatted output tables with `prettytable`
   - Calculates and displays subtotals and grand total

## Installation

### Prerequisites

- Rust 1.70+ (2024 edition)
- PDFium library for PDF processing

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/reimburser.git
cd reimburser

# Install dependencies (macOS/Linux)
./setup.sh

# Build the project
cargo build --release

# The binary will be available at target/release/reimburser
```

### Using Nix (Optional)

If you have Nix installed, you can use the provided flake for a reproducible development environment:

```bash
nix develop --extra-experimental-features nix-command --extra-experimental-features flakes
cargo build --release
```

## Usage

### Basic Usage

Calculate reimbursable trips between your home station (e.g., Hilversum) and work stations (e.g., Amsterdam Centraal, Amsterdam Zuid):

```bash
reimburser \
  --input invoice.pdf \
  --from-ns Hilversum \
  --to-ns "Amsterdam Centraal" \
  --to-ns "Amsterdam Zuid"
```

### Real-World Example

Based on your commute pattern (Hilversum ↔ Amsterdam):

```bash
./target/release/reimburser \
  --input ~/Downloads/ns-invoice.pdf \
  --from-ns Hilversum \
  --to-ns "Amsterdam Centraal" \
  --to-ns "Amsterdam Zuid" \
  --from-gvb "Centraal Station" \
  --to-gvb "Paleisstraat" \
  --to-gvb "Rokin" \
  --from-gvb "Station Zuid"
```

This command will:
1. Parse your NS invoice PDF
2. Find all trips between Hilversum and Amsterdam stations
3. Include GVB trips within Amsterdam (e.g., from Centraal Station to Paleisstraat/Rokin)
4. Filter for workdays only
5. Calculate total reimbursable amount

### With GVB Trips

If your invoice includes GVB (Amsterdam public transport) trips:

```bash
reimburser \
  --input invoice.pdf \
  --from-ns Hilversum \
  --to-ns "Amsterdam Centraal" \
  --from-gvb "Centraal Station" \
  --to-gvb "Science Park"
```

### Multiple Home/Work Stations

You can specify multiple departure and arrival stations:

```bash
reimburser \
  --input invoice.pdf \
  --from-ns Hilversum \
  --from-ns Utrecht \
  --to-ns "Amsterdam Centraal" \
  --to-ns "Amsterdam Zuid" \
  --to-ns "Amsterdam Sloterdijk"
```

### Example Output

```
+----------+------------+--------------------+--------------------+-------+
| Provider | Date       | From               | To                 | Price |
+----------+------------+--------------------+--------------------+-------+
| NS       | 2025-01-15 | Hilversum          | Amsterdam Centraal | 5.60  |
| NS       | 2025-01-15 | Amsterdam Centraal | Hilversum          | 5.60  |
| NS       | 2025-01-16 | Hilversum          | Duivendrecht       | 3.80  |
| NS       | 2025-01-16 | Duivendrecht       | Amsterdam Zuid     | 2.90  |
| NS       | 2025-01-16 | Amsterdam Zuid     | Duivendrecht       | 2.90  |
| NS       | 2025-01-16 | Duivendrecht       | Hilversum          | 3.80  |
| GVB      | 2025-01-16 | Centraal Station   | Science Park       | 2.60  |
| GVB      | 2025-01-16 | Science Park       | Centraal Station   | 2.60  |
+----------+------------+--------------------+--------------------+-------+

NS subtotal:  25.00
GVB subtotal: 5.20
-------------------
Grand total: 30.20
```

## How It Works

1. **PDF Parsing**: The tool reads your NS invoice PDF (downloadable from https://www.ns.nl/mijnns#/betaaloverzicht)
2. **Trip Extraction**: Uses regex patterns to extract individual trips from the PDF text
3. **Station Matching**: Identifies departure and arrival stations from the complete station database
4. **Chain Detection**: Automatically detects multi-leg journeys (e.g., Hilversum → Duivendrecht → Amsterdam Zuid)
5. **Filtering**: Applies workday and station filters to identify reimbursable trips
6. **Calculation**: Sums up all qualifying trips and presents them in a formatted table

## Command-Line Arguments

| Argument | Description | Example |
|----------|-------------|---------|
| `-f, --input` | Path to NS invoice PDF file | `--input invoice.pdf` |
| `--from-ns` | NS departure station(s) | `--from-ns Hilversum` |
| `--to-ns` | NS arrival station(s) | `--to-ns "Amsterdam Centraal"` |
| `--from-gvb` | GVB departure station(s) (optional) | `--from-gvb "Centraal Station"` |
| `--to-gvb` | GVB arrival station(s) (optional) | `--to-gvb "Science Park"` |

## Dependencies

- `anyhow` - Error handling
- `chrono` - Date/time parsing and manipulation
- `clap` - Command-line argument parsing
- `pdfium-render` - PDF text extraction
- `prettytable` - Formatted table output
- `regex` - Pattern matching for invoice parsing

## Data Sources

Station data is compiled from GTFS (General Transit Feed Specification) data available at https://gtfs.ovapi.nl/

To update station lists:
```bash
# NS stations
cat gtfs-openov-nl/stops.txt | grep -E '[0-9]+.+0,stoparea:[0-9]+,,,$' | grep -Ev '\[|\]' | awk -F ',' '{print $3}' | sort | uniq

# GVB stations
cat gtfs-openov-nl/stops.txt | grep -E '^[0-9]+,,"Amsterdam, .+".+,0,,,[0-9]?,?$' | awk -F ',' '{print $4}' | awk '{sub(" ", "", $0)}1' | tr -d '"' | sort | uniq
```

## License

This project is open source. Please check the repository for license details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Notes

- Only weekday trips (Monday-Friday) are included in calculations
- The tool automatically handles multi-leg journeys on the same day
- Station names must match exactly as they appear in the invoice
- Free trips (€0.00) are automatically excluded from calculations
- PDFium library is included for macOS; other platforms may need to download it separately
