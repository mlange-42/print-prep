..\target\release\pprep ^
  --input "../test_data/*.png" "../test_data/*.jpg" ^
  --cmd ^
  scale ^
    --output "../test_data/out/*-scale-tiny.png" ^
    --scale 10%% ^
	--incremental
pause