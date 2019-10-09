awk '{for(i=1;i<=NF;i++) count[$i]++} END{ for(patten in count) printf("%s %d\n",patten,count[patten])}' words.txt |sort -n -r -k2,2
