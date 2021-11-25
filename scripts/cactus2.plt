#!/usr/bin/gnuplot -persist
set term pdfcairo enhance
set xlabel "Number of solved instances"
set ylabel "Time (s)"
set output './figure/with_propagate.pdf'
csv="benchmark2.csv"

cactus(method)=sprintf("< echo 0; grep %s %s | cut -d',' -f 3 | sort -n", method, csv)

set key top left
set style data linespoints
set pointsize 0.4
set style increment user
plot \
cactus("^my_solver_with_propagate,") title "my solver with propagate",\
cactus("^minisat,") title "minisat"