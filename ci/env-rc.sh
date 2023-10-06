#!/bin/bash


# Set var and validate return value is not empty
CI_TAG="$(git show-ref --tags | grep "$(git rev-parse HEAD)" | awk -F/ '{print $3}')"
# count amount of characters in the variable
validate=${#CI_TAG}
if [[ $validate -lt 1 ]]
then
    CI_TAG=$(git tag --points-at HEAD)
    validate=${#CI_TAG}
    if [[ $validate -lt 1 ]]
    then
        CI_TAG=""
    fi
fi

pre_release=""

# See if any tag denotes a pre-release
for p_tag in $CI_TAG
do
  if [[ "$p_tag" =~ rc_.* ]]
  then
    # Printing value for return
    pre_release="$p_tag"
  fi
done

echo "Pre release" $pre_release


source ./ci/env.sh
