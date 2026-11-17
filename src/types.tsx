
export interface CurvePoint {
    hand_id: string,
    date: string,
    net: number,
    net_bb: number,
    cumulative: number,
    cumulative_bb: number,
    cumulative_showdown: number,
    cumulative_showdown_bb: number,
    cumulative_nonshowdown: number,
    cumulative_nonshowdown_bb: number,
}
