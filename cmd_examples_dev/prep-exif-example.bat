..\target\release\pprep ^
  --input "../test_data/*.jpg" ^
  --cmd ^
  prep ^
    --output "../test_data/out/*-exif.png" ^
    --format 10cm/15cm ^
    --padding 5mm ^
    --margins 5mm ^
    --cut-marks ./1mm ^
    --exif "{F/2}, {Exp}, ISO {ISO}, {F}" ^
    --test-pattern 15px/3px ^
    --dpi 150
pause