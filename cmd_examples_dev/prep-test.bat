..\target\release\pprep ^
  --input ../test_data/*.png ^
  --debug ^
  prep ^
    --output ../test_data/out/*-prep.png ^
    --format 10cm/15cm ^
	--padding 5mm/5mm/1cm/5mm ^
	--margins 5mm ^
	--border 2px/2px/5px/2px ^
	--border-color red ^
	--cut-marks ./1mm ^
	--dpi 300
pause