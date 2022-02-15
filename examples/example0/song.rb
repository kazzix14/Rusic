require 'jungru'
require 'jungru/support'

j = ::Jungru::Piece.new

j.instrument :inst1 do |i|
  i.before_each_note do |i, n|
    #i.offset -4.0
  end
  i.signal do |i, n, l, t|
    i.out (
        sin(n[:freq], t) \
        + \
        sin(n[:freq] * 1.2 + 0.1 * sin(0.2, t), t) \
        + \
        sin(n[:freq] * 1.5 + 0.1 * sin(0.1, t), t) * (0.8 + 0.2 * sin(0.13, t))
      )
          .amp(adsr(2.0, 4.0, l, 0.4 , 4.0, t))
          .amp(0.1)
  end
end

j.instrument :inst0 do |i|
  i.signal do |i, n, l, t|
    if t < 2 * l
      i.out Math.sin(n[:f] * 2.0 * Math::PI * t) * n[:v]
    else
      i.out nil
    end
  end
end

j.instrument :kick do |i|
  i.signal do |i, n, l, t|
    i.out \
      (sin(240.0 * adsr(0.01, 0.01, 0.1, 0.3, 0.4, t, last: 0.01), t)
        .amp(adsr(0.01, 0.05, 0.1, 0.3, 0.5, t)) + \
      sin(2400.0 * adsr(0.001, 0.005, 0.0, 0.01, 0.1, t, last: 0.01), t)
        .amp(adsr(0.001, 0.005, 0.0, 0.3, 0.05, t)))
      .amp(0.3)

  end
end

j.meta do |m|
  m.bpm 175.0
  m.composite \
  '
    a
  '
end

j.track :pad, :inst1 do |t|
  t.symbols do |s, i|
    { freq: 110.0 *  (1.0 + i.to_f / 10.0) }
  end

  t.section :a do |s|
    s.division 1, 1

    str = ""
    (0..8).map do |_|
      str << alphanumeric.sample.to_s
    end
    
    p str

    s.sheet str
  end
end

j.track :kick , :kick do |t|
  t.symbol :a, {}
  t.section :a do |s|
    s.division 1, 4
    
    s.sheet \
    '
      aaaa aaaa
      aaaa aaaa

      aaaa aaaa
      aaaa aaaa
    '
  end
end

j.gen :wav, 'out.wav'