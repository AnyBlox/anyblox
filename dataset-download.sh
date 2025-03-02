USAGE="Usage: command OUTPUT_DIRECTORY"

if [ $# != 1 ]; then
    echo $USAGE
    exit 1;
fi

DIR=$1

mkdir -p $DIR
cd $DIR

for i in $(seq 1 2); do
    wget "https://event.cwi.nl/da/PublicBIbenchmark/Taxpayer/Taxpayer_${i}.csv.bz2"
    wget "https://event.cwi.nl/da/PublicBIbenchmark/CommonGovernment/CommonGovernment_${i}.csv.bz2"
done
echo "Extracting Taxpayer..."
bzip2 -d Taxpayer_*.csv.bz2
echo "Extracting CommonGovernment..."
bzip2 -d CommonGovernment_*.csv.bz2

wget https://datasets.clickhouse.com/hits_compatible/hits.parquet
mkdir ClickBench
mv hits.parquet ClickBench/

wget https://opendata.cern.ch/record/4900/files/B2HHH_MagnetUp.root