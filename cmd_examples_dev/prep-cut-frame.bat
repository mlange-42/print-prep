..\target\release\pprep ^
  --input "../test_data/*.png" ^
  --cmd ^
  prep ^
    --output "../test_data/out/*-frame.png" ^
    --format 10cm/15cm ^
	--padding 5mm ^
	--margins 2mm ^
	--cut-frame ./2mm ^
	--dpi 300
pause