..\target\release\pprep ^
  --input ../test_data/*.png ^
  --debug ^
  prep ^
    --output ../test_data/out/*-frame.png ^
    --format 15cm/10cm ^
    --framed-size 12cm/6cm ^
	--padding 5mm/5mm/5mm/5mm ^
	--cut-frame ./. ^
	--dpi 300
pause