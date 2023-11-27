set datafile separator "\t"
set xdata time
set timefmt "%s" # Assumes timestamps are in Unix epoch format
set format x "%I:%M" # Format the x-axis labels
set xlabel "Time"
set ylabel "Implied Probability"
set yrange [0:1]
set y2label "Handicap"
set y2tics
set ytics ("0.0" 0, "0.1" 0.1, "0.2" 0.2, "0.3" 0.3, "0.4" 0.4, "0.5" 0.5, "0.6" 0.6, "0.7" 0.7, "0.8" 0.8, "0.9" 0.9, "1.0" 1.0)
set grid ytics lt 0 lw 1 lc rgb "#888888"
#set logscale y
set title "Implied Probability + Handicap over Time"
set terminal png size 1280,720
set output output_file
plot \
    input_file using ($1+(-5*3600)):2 with lines title "Probability" axis x1y1, \
    input_file using ($1+(-5*3600)):4 with lines title "Handicap" axis x1y2
