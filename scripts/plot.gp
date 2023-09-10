set datafile separator ","
set xdata time
set timefmt "%s"  # Assumes timestamps are in Unix epoch format
set format x "%h:%M"  # Format the x-axis labels
set xlabel "Time"
set ylabel "Implied Probability"
set title "Implied Probability over Time"
set terminal pngcairo
set output "/tmp/output.png"
plot "/tmp/extracted.csv" using 1:2 with lines title "Probability"
