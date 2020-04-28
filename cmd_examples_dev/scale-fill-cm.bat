..\target\release\pprep ^
  --input "../test_data/*.png" "../test_data/*.jpg" ^
  --cmd ^
  scale ^
    --output "../test_data/out/*-scale.png" ^
    --size 2cm/5cm ^
	--mode fill ^
	--bg blue
pause