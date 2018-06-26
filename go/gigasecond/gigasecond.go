package gigasecond

import "time"

// AddGigasecond add 10^9 seconds to time.Time
func AddGigasecond(t time.Time) time.Time {
	val := time.Second * 1000000000
	return t.Add(time.Duration(val))
}
