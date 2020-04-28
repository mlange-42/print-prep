..\target\release\pprep ^
  --input "../test_data/*.png" "../test_data/*.jpg" ^
  --cmd ^
  scale ^
    --output "../test_data/out/*-scale-percent.png" ^
    --scale 25%%/50%% ^
	--mode stretch
pause