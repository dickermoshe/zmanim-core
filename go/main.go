package main

import (
	"fmt"
	"time"

	"github.com/dickermoshe/zmanim-core/go/zmanim_core"
)

func main() {
	msSinceEpoch := time.Now().UnixMilli()
	location := zmanim_core.NewGeolocation(31.768319, 35.213711, 0)
	calendar := zmanim_core.NewAstronomicalCalendar(msSinceEpoch, *location)
	sunrise := calendar.GetSunrise()
	// Parse the sunrise time to a time.Time ms since epoch
	sunriseTime := time.UnixMilli(*sunrise)
	fmt.Println("Sunrise time:", sunriseTime)
}
