require 'jungru'
require 'jungru/support'
require 'debug'


#j.instrument :inst1 do |i|
#  i.before_each_note do |i, n|
#    #i.offset -4.0
#  end
#  i.signal do |i, n, l, t|
#    i.out (
#        sin(n[:freq], t) \
#        + \
#        sin(n[:freq] * 1.2 + 0.1 * sin(0.2, t), t) \
#        + \
#        sin(n[:freq] * 1.5 + 0.1 * sin(0.1, t), t) * (0.8 + 0.2 * sin(0.13, t))
#      )
#          .amp(adsr(2.0, 4.0, l, 0.4 , 4.0, t))
#          .amp(0.1)
#  end
#end
#
#j.instrument :inst0 do |i|
#  i.signal do |i, n, l, t|
#    if t < 2 * l
#      i.out Math.sin(n[:f] * 2.0 * Math::PI * t) * n[:v]
#    else
#      i.out nil
#    end
#  end
#end

j = ::Piece.new

j.instrument :pluck do |i|
  i.before_each_note do |i, n|
    i.save :osc0, ::Oscillator.new(0.0)
    i.save :osc1, ::Oscillator.new(0.1)
    i.save :osc2, ::Oscillator.new(0.2)
    i.save :osc3, ::Oscillator.new(0.3)
    i.save :osc4, ::Oscillator.new(0.4)
    i.save :osc5, ::Oscillator.new(0.5)

    i.save :osc0r, ::Oscillator.new(1.0)
    i.save :osc1r, ::Oscillator.new(1.1)
    i.save :osc2r, ::Oscillator.new(1.2)
    i.save :osc3r, ::Oscillator.new(1.3)
    i.save :osc4r, ::Oscillator.new(1.4)
    i.save :osc5r, ::Oscillator.new(1.5)
    #i.offset rand * 1.0
  end

  i.signal do |i, n, l, t, dt|

    a = 0.1
    d = 0.4
    s = 0.6
    r = 0.3
    if t < a+d+s+r && !n[:f].nil?
      osc0 = i.load :osc0
      osc1 = i.load :osc1
      osc2 = i.load :osc2
      osc3 = i.load :osc3
      osc4 = i.load :osc4
      osc5 = i.load :osc5
      i.left \
        (
          osc0.sq(n[:f][0] + rand, dt) +
          osc1.sq(n[:f][1] + rand, dt) +
          osc2.saw(n[:f][2] + rand + 2.0 + Math.cos(4.0 * t), dt) +
          osc3.saw(n[:f][3] + rand + Math.sin(5.0 * t), dt) +
          osc4.sin((n[:f][0] + rand) / 2, dt) +
          osc5.sin((n[:f][0] + rand) / 4, dt)
        )
          .amp(0.010)
          .amp(adsr(a, d, s, 0.03, r, t))
      i.save :osc0, osc0
      i.save :osc1, osc1
      i.save :osc2, osc2
      i.save :osc3, osc3
      i.save :osc4, osc4
      i.save :osc5, osc5

      osc0 = i.load :osc0r
      osc1 = i.load :osc1r
      osc2 = i.load :osc2r
      osc3 = i.load :osc3r
      osc4 = i.load :osc4r
      osc5 = i.load :osc5r
      i.right \
        (
          osc0.sq(n[:f][0] + rand, dt) +
          osc1.sq(n[:f][1] + rand, dt) +
          osc2.saw(n[:f][2] + rand + Math.cos(4.0 * t), dt) +
          osc3.saw(n[:f][3] + rand + 3.0 + Math.sin(5.0 * t), t) +
          osc4.sin((n[:f][0] + rand) / 2, dt) +
          osc5.sin((n[:f][0] + rand) / 4, dt)
        )
          .amp(0.010)
          .amp(adsr(a, d, s, 0.03, r, t))
      i.save :osc0r, osc0
      i.save :osc1r, osc1
      i.save :osc2r, osc2
      i.save :osc3r, osc3
      i.save :osc4r, osc4
      i.save :osc5r, osc5
    end
  end
end

j.instrument :hat do |i|
  i.signal do |i, n, l, t, dt|
    if !n[:p].nil?
      i.out \
        (rand * 2.0 - 1.0)
          .amp(adsr(0.01, 0.25, 0.0, 0.2, 0.1, t))
          .amp(0.015)
    end
  end
end

j.instrument :arp do |i|
  i.init do |i| 
    i.save :fh, 0
  end
  i.before_each_note do |i, n|
    i.save :o0, ::Oscillator.new(0.0)
    i.save :o1, ::Oscillator.new(0.0)
    i.save :o2, ::Oscillator.new(0.0)
    i.save :pan, rand < 0.5

    fh = i.load :fh

    i.save :f, n[:f][fh]

    fh += 1
    fh = 0 if 3 < fh

    i.save :fh, fh
  end

  i.signal do |i, n, l, t, dt|
    if t < 0.1
      pan = i.load :pan
      o0 = i.load :o0
      o1 = i.load :o1
      o2 = i.load :o2
      f = i.load :f
      
      if pan
        i.left \
          (o0.sin(f, dt) + o1.sin(f * 1.5, dt) + o2.sin(f * 2.0, dt))
            .amp(adsr(0.01, 0.2, 0.05, 0.2, 0.1, t))
            .amp(0.02)
            
        i.right 0.0
      else
        i.right \
          (o0.sin(f, dt) + o1.sin(f * 1.5, dt) + o2.sin(f * 2.0, dt))
            .amp(adsr(0.01, 0.2, 0.05, 0.2, 0.1, t))
            .amp(0.02)
        i.left 0.0
      end
      i.load :o0, o0
      i.load :o1, o1
      i.load :o2, o2
    end
  end
end

j.instrument :kick do |i|
  i.before_each_note do |i, n|
    i.save :osc0, ::Oscillator.new(0.0)
    i.save :osc1, ::Oscillator.new(0.1)
  end

  i.signal do |i, n, l, t, dt|
    osc0 = i.load :osc0
    osc1 = i.load :osc1

    i.out \
      (
        osc0.sin(150.0 * adsr(0.01, 0.01, 0.1, 0.3, 0.4, t, last: 0.01), dt)
          .amp(adsr(0.01, 0.05, 0.1, 0.3, 0.5, t)) + \
        osc1.sin(3400.0 * adsr(0.001, 0.010, 0.0, 0.01, 0.1, t, last: 0.01), dt)
          .amp(adsr(0.001, 0.005, 0.0, 0.3, 0.05, t))
          .amp(0.4)
      ).amp(0.1)

    i.save :osc0, osc0
    i.save :osc1, osc1
  end
end

j.meta do |m|
  m.bpm 123.0
  # バグってる. セクションの長さを, noteのステート計算に着かなきゃいけない
  m.composite 'b b '
end

chords = 90.0.minor_scale.chords
chords = chords.map { |c| ->() {
    c.omit([3, 5].sample)
      .add(7)
      .add([9, 11].sample)
  }}

f = -> (s) {"#{s}..#{s} ..#{s}."}
#f2 = ->() { [%w(3).sample, %w(1 3 2 4 6).sample, %w(5 7).sample, "5"].map { |sym| f.call(sym) }.join(' ') }
f2 = ->() { [%w(3).sample, %w(1 3 2 4 6).sample, %w(5 7).sample, "5"].join(' ') }
prog = [f2.call, f2.call, f2.call, f2.call].join("     ")
prog_char_only = prog.chars
prog_char_only.delete(" ")
prog_arp = prog_char_only.map { |c| (0..8).map { |_| c } }.join('')
prog_pluck= prog.chars.map{ |s| f.call(s) unless s == ' '}.compact.join(' ')

j.track :pluck, :pluck do |t|
  (1..7).each do |i|
    t.symbol :"#{i}", chords[i-1].call().note || { }
  end
  t.symbol :'.', { }
  
  t.section :a do |s|
    s.division 1, 16
    s.length 8, 1
    
    s.sheet prog_pluck
  end

  t.section :b do |s|
    s.division 1, 16
    s.length 8, 1
    
    s.sheet prog_pluck
  end
end

j.track :arp, :arp do |t|
  (1..7).each do |i|
    t.symbol :"#{i}", chords[i-1].call().note || { }
  end
  
  t.section :a do |s|
    s.division 1, 16
    s.length 8, 1
    
    s.sheet ' '
  end

  t.section :b do |s|
    s.division 1, 16
    s.length 8, 1
    
    s.sheet prog_arp
  end
end

j.track :kick, :kick do |t|
  t.section :a do |s| 
    s.division 1, 4
    s.length 8, 1
    s.symbol :a, { }
    s.sheet 'a'
  end

  t.section :b do |s| 
    s.division 1, 4
    s.length 8, 1
    s.symbol :a, { }
    s.sheet 'a'
  end
end

j.track :hat, :hat do |t|
  t.section :a do |s| 
    s.division 1, 16
    s.length 8, 1
    s.symbol :'.', { }
    s.symbol :a, { p: true }
    s.sheet '
    ..a. ..a. ..a. ..a.
    ..a. ..a. ..a. ..a.
    '
  end

  t.section :b do |s| 
    s.division 1, 16
    s.length 8, 1
    s.symbol :'.', { }
    s.symbol :a, { p: true }
    s.sheet '
    ..a. ..a. ..a. ..a.
    ..a. ..a. ..aa ..a.
    '
  end
end

j.gen :wav, 'out.wav'

#
#j.track :t2, :kick do |t|
#  t.symbols do |k, i|
#    { f: 440.0.eq12.sample * rand }
#  end
#  
#  t.section :a do |s|
#    s.division 1, 8
#    s.length 8, 1
#    
#    str = ""
#    
#    for _ in 0...32
#      str << alphanumeric.sample.to_s 
#    end
#    
#    s.sheet str
#  end
#end
#
#j.track :t3, :kick do |t|
#  t.symbols do |k, i|
#    { f: 440.0.eq12.sample * rand }
#  end
#  
#  t.section :a do |s|
#    s.division 1, 8
#    s.length 8, 1
#    
#    str = ""
#    
#    for _ in 0...32
#      str << alphanumeric.sample.to_s
#    end
#    
#    s.sheet str
#  end
#end
