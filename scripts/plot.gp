set datafile separator ","
set xdata time
set timefmt "%s"  # Assumes timestamps are in Unix epoch format
set format x "%I:%M"  # Format the x-axis labels
set xlabel "Time"
set ylabel "Implied Probability"
set yrange [0:1]
set ytics ("0.0" 0, "0.1" 0.1, "0.2" 0.2, "0.3" 0.3, "0.4" 0.4, "0.5" 0.5, "0.6" 0.6, "0.7" 0.7, "0.8" 0.8, "0.9" 0.9, "1.0" 1.0)
set grid ytics lt 0 lw 1 lc rgb "#888888"
#set logscale y
set title "Implied Probability over Time"
set terminal png size 1280,720
set output output_file
plot input_file using 1:2 with lines title "Probability"
