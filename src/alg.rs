/* Basically a list of stars (which are 'on'(true) or 'off'(false)) */
pub type Constellation = Vec<bool>;

fn rule1(slice: &[bool]) -> bool {
    /* If the next one is lit, and the other ones dark */
    slice[1] && slice[2..].iter().all(|&x| x == false)
}

fn rule2(slice: &[bool]) -> bool {
    /* If its the last element of the list */
    slice.len() == 1
}

fn canswitch(slice: &[bool]) -> bool {
    rule2(slice) || rule1(slice)
}

/* Switch the star to the correct state according the the rules */
fn switchstar(slice: &mut [bool], count: &mut usize) {
    if !canswitch(slice) {
        let len = slice.len();

        /* If the next one isn't lit, turn it on */
        if !slice[1] {
            switchstar(&mut slice[1..], count);
        }
        /* Turn everything else off */
        for id in 2..len {
            if slice[id] {
                switchstar(&mut slice[id..], count);
            }
        }
    }
    /* Will crash if the above didn't made switch possible */
    assert!(canswitch(slice));

    /* Perform the star on/off switch */
    slice[0] = !slice[0];
    *count += 1;
}

pub fn starify(s: String) -> Constellation {
    /* Translate a string to a constellation */
    s.chars().map(|x| x.to_digit(2).unwrap() != 0).collect()
}

pub fn resolve(mut constellation: Constellation, target: Constellation, len: usize) -> usize {
    let mut count: usize = 0;

    /* Crawl the constellation to optain `target` from `constellation` */
    for id in 0..len {
        /* Ignore already well-lit stars */
        if constellation[id] == target[id] {
            continue;
        };

        /* Pass the wanted slice to switch the state of the `star`
         * (which is the first element of the slice) */
        let (_, star) = constellation.split_at_mut(id);
        switchstar(star, &mut count);
    }

    /* Will crash if we didn't solve it */
    assert!(constellation == target);
    count
}
