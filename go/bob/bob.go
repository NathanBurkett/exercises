package bob

import (
	"regexp"
	"strings"
)

type Remark struct {
	value string
}

// Hey Interpret Bob's response based upon remark
func Hey(remark string) string {
	r := &Remark{value: strings.Trim(remark, " ")}

	return r.respond()
}

func (r *Remark) isEmpty() bool {
	return r.value == ""
}

func (r *Remark) hasAlphaCharacters() bool {
	match, _ := regexp.MatchString("(^.*[a-zA-Z].*?$)", r.value)
	return match
}

func (r *Remark) isOnlyWhitespace() bool {
	match, _ := regexp.MatchString("^( |\\t|\\n|\\r)+$", r.value)
	return match
}

func (r *Remark) endsWithQuestionMark() bool {
	match, _ := regexp.MatchString("(^.*\\?$)", r.value)
	return match
}

func (r *Remark) hasNoLowercaseChars() bool {
	match, _ := regexp.MatchString("(^.*[a-z].*?$)", r.value)
	return !match
}

func (r *Remark) respond() string {
	if r.isEmpty() || r.isOnlyWhitespace() {
		return "Fine. Be that way!"
	}

	isQuestion := r.endsWithQuestionMark()

	if r.hasAlphaCharacters() {
		isYelling := r.hasNoLowercaseChars()

		if isQuestion && isYelling {
			return "Calm down, I know what I'm doing!"
		}

		if isQuestion {
			return "Sure."
		}

		if isYelling {
			return "Whoa, chill out!"
		}
	}

	if isQuestion {
		return "Sure."
	}

	return "Whatever."
}
