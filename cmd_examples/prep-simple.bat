..\pprep ^
  --input "../test_data/*.png" ^
  --cmd ^
  prep ^
    --output "../test_data/out/*-simple.png" ^
    --format 10cm/15cm ^
	--padding 5mm ^
	--margins 5mm ^
	--cut-marks ./1mm ^
    --dpi 90
pause