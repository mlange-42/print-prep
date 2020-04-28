..\target\release\pprep ^
  --input "../test_data/*.png" "../test_data/*.jpg" ^
  --cmd ^
  prep ^
    --output "../test_data/out/*-marks.png" ^
    --format 10cm/15cm ^
	--padding 5mm ^
	--margins 2mm ^
	--cut-marks ./1mm ^
	--dpi 300
pause