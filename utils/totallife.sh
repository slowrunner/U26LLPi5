#!/bin/bash
#
# totallife.sh    print total hours and sessions of life in life.log
#
# requires bc  (sudo apt-get install bc)
#
# Counted Keys:
#   Playtimes:           ": success "
#   Sessions:            "- boot -"              ( "\- boot \-" search string )
#   Safety shutdowns:    "SAFETY SHUTDOWN"
#
# Summed Keys:
#   execution:
#   nap for

homedir='U26LLPi5'
username='ubuntu'

echo "(Cleaning life.log first)"
/home/$username/$homedir/plib/cleanlifelog.py
echo " "
fn="/home/$username/$homedir/logs/life.log"
totalAwake=`(awk -F'execution:' '{sum+=$2}END{print sum;}' $fn)`
totalNaps=`(awk -F'nap for' '{sum+=$2}END{print sum;}' $fn)`
totalLife=`(echo "scale=1; ($totalAwake + $totalNaps)" | bc)`
# weirdness without +'' at end, see ls appended to lastDockingStr??

echo "*** ${homedir} TOTAL LIFE STATISTICS ***"
echo "Total Awake:  " $totalAwake " hrs"
echo "Total Naps:    " $totalNaps " hrs"
echo "Total Life:   " $totalLife " hrs (since August 2025)"
booted=`(grep -c "\- boot \-" $fn)`
echo "Sessions (boot): " `(grep -c "\- boot \-" $fn)`
aveSession=`(echo "scale=1; ($totalAwake / $booted)" | bc -l)`
echo "Average Session: " $aveSession "hrs"
safetyShutdowns=`(grep -c "SAFETY SHUTDOWN" $fn)`
echo "Safety Shutdowns: " $safetyShutdowns 
