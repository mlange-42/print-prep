..\target\release\pprep ^
  --input "../test_data/*.jpg" ^
  --cmd ^
  prep ^
    --output "../test_data/out/*-simple.png" ^
    --format 10cm/15cm ^
	--padding 5mm ^
	--margins 5mm ^
	--cut-marks ./1mm ^
	--dpi 150
pause