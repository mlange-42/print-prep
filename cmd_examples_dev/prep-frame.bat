..\target\release\pprep ^
  --input ../test_data/*.png ^
  --debug ^
  prep ^
    --output ../test_data/out/*-frame.png ^
    --format 10cm/15cm ^
	--padding 5mm/5mm/1cm/5mm ^
	--margins 2mm ^
	--cut-frame ./2mm ^
	--dpi 300
pause