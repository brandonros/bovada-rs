set datafile separator "\t"
set xdata time
set timefmt "%s" # Assumes timestamps are in Unix epoch format
set format x "%I:%M" # Format the x-axis labels
set xlabel "Time"
set ylabel "American Odds"
set yrange [-500:500]
set y2label "Handicap"
set y2tics
set ytics
set grid ytics lt 0 lw 1 lc rgb "#888888"
#set logscale y
set title "American Odds + Handicap over Time"
set terminal png size 1280,720
set output output_file
plot \
    input_file using ($1+(-5*3600)):3 with lines title "Probability" axis x1y1, \
    input_file using ($1+(-5*3600)):4 with lines title "Handicap" axis x1y2
