..\target\release\pprep ^
  --input "../test_data/*.png" "../test_data/*.jpg" ^
  --cmd ^
  prep ^
    --output "../test_data/out/*-border-sides.png" ^
    --format 10cm/15cm ^
	--padding 5mm/5mm/1cm/5mm ^
	--border 1px/1px/5px/1px ^
	--border-color red ^
	--margins 2mm ^
	--cut-frame ./. ^
	--dpi 300
pause