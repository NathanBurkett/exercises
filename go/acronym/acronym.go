package acronym

import (
	"bytes"
	"strings"
)

type Splitter struct {
	raw       string
	cond      runeCond
	result    *bytes.Buffer
	processed bool
	_         struct{}
}

type runeCond func(rune) bool

func NewSplitter(raw string, cond runeCond) *Splitter {
	return &Splitter{
		raw:    raw,
		cond:   cond,
		result: new(bytes.Buffer),
	}
}

// Abbreviate Turn string into acronym by first letter of words split by " " and "-"
func Abbreviate(s string) string {
	return NewSplitter(s, func(r rune) bool {
		return r == ' ' || r == '-'
	}).Run().String()
}

// Run Split the string
func (s *Splitter) Run() *Splitter {
	return s.split()
}

func (s *Splitter) split() *Splitter {
	if s.processed {
		return s
	}

	for _, item := range strings.FieldsFunc(s.raw, s.cond) {
		s.result.WriteString(strings.ToUpper(item[:1]))
	}

	s.processed = true

	return s
}

// String Get the result string from the split operation
func (s *Splitter) String() string {
	return s.result.String()
}
