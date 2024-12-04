
new-day day:
    @cp templates/day.rs src/day{{day}}.rs
    @sed -i '0,/^$/s/^$/mod day{{day}};\n/' src/lib.rs 
