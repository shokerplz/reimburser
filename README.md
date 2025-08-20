# Build

execute setup.sh
cargo build

GTFS we can get from here <https://gtfs.ovapi.nl/>
HOW TO GET OV-NS STOPS: cat gtfs-openov-nl/stops.txt | grep -E '[0-9]+.+0,stoparea:[0-9]+,,,$' | grep -Ev '\[|\]' | awk -F ',' '{print $3}' | sort | uniq
HOW TO GET OV-GVB STOPS: cat gtfs-openov-nl/stops.txt | grep -E '^[0-9]+,,"Amsterdam, .+".+,0,,,[0-9]?,?$' | awk -F ',' '{print $4}' | awk '{sub(" ", "", $0)}1' | tr -d '"' | sort | uniq  | wc -l
