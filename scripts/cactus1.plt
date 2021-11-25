#!/usr/bin/gnuplot -persist
set term pdfcairo enhance
set xlabel "Number of solved instances"
set ylabel "Time (s)"
set output './figure/simple_solver.pdf'
csv="benchmark.csv"

cactus(method)=sprintf("< echo 0; grep %s %s | cut -d',' -f 3 | sort -n", method, csv)

set key top left
set style data linespoints
set pointsize 0.9
set style increment user
plot \
cactus("^simple_minisat,") title "simple minisat", \
cactus("^my_simple_solver,") title "my simple solver"