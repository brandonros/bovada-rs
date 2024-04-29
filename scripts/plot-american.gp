yrange_min = -750
yrange_max = 750
y2range_min = -25
y2range_max = 50

# Timezone adjustment
timezone_offset = -4 * 3600  # Offset in seconds

set datafile separator "\t"
set xdata time
set timefmt "%s" # Assumes timestamps are in Unix epoch format
set format x "%I:%M" # Format the x-axis labels
set xlabel "Time"
set ylabel "American Odds"
set yrange [yrange_min:yrange_max]
set ytics
set grid ytics lt 0 lw 1 lc rgb "#888888"
#set logscale y
set terminal png size 1280,720
set output output_file

if (strstrt(show_y2, "true") > 0) {
    set title "American Odds + Handicap over Time"
    set y2label "Handicap"
    set y2tics
    #set y2range [y2range_min:y2range_max]
    plot \
        input_file using ($1 + timezone_offset):3 with lines title "Probability" axis x1y1, \
        input_file using ($1 + timezone_offset):4 with lines title "Handicap" axis x1y2
} else {
    set title "American Odds over Time"
    plot \
        input_file using ($1 + timezone_offset):3 with lines title "Probability" axis x1y1
}
