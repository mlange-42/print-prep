..\pprep ^
  --input "../test_data/*.png" "../test_data/*.jpg" ^
  --debug ^
  prep ^
    --output "../test_data/out/*-exif.png" ^
    --format 10cm/15cm ^
    --padding 5mm ^
    --margins 5mm/5mm/10mm/5mm ^
    --cut-marks ./1mm ^
    --exif "{F/2}, {Exp}, ISO {ISO}, {F}" ^
    --exif-size 25px ^
    --test-pattern 25px/5px ^
    --dpi 300
pause